#!/usr/bin/env bash
set -euo pipefail

# Determine doc directory as the script's own directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOC_DIR="$SCRIPT_DIR"
INDEX_FILE="$DOC_DIR/0.0.index.md"

tmpdir=$(mktemp -d 2>/dev/null || mktemp -d -t 'updnav')
trap 'rm -rf "$tmpdir"' EXIT

titles_tsv="$tmpdir/titles.tsv"
chapters_tsv="$tmpdir/chapters.tsv"

# Build titles mapping from index (plus Summary for index itself)
{
  printf '%s\t%s\n' '0.0.index.md' 'Summary'
  awk '
    BEGIN{FS="[()\[\]]"}
    $0 ~ /^[[:space:]]*[0-9]+\.[0-9]+[[:space:]]+\[[^]]+\]\([^)]*\.md\)/ {
      # Example line: 1.1 [Introduction](1.1.introduction.md)
      # After splitting by ()[]: fields around brackets are in $0; link text in $2; link target in $3
      # But FS splits into: pre [ text ] ( link ) post => $2 is text, $3 is link
      title=$2; file=$3;
      print file"\t"title
    }
  ' "$INDEX_FILE" 2>/dev/null || true
} > "$titles_tsv"

# Build chapter roman + names from index headers like: #### I - Project
awk '
  $0 ~ /^[[:space:]]*####[[:space:]]+[IVXLCDM]+[[:space:]]*-[[:space:]]*.+/ {
    line=$0
    sub(/^[[:space:]]*####[[:space:]]*/, "", line)
    roman=line; sub(/[[:space:]]*-.*/, "", roman); gsub(/[[:space:]]/, "", roman)
    name=line; sub(/^.*-[[:space:]]*/, "", name)
    # Convert roman to int
    split("I V X L C D M", sym, " "); split("1 5 10 50 100 500 1000", val, " ")
    n=split(roman, arr, "")
    total=0; prev=0
    for(i=n;i>=1;i--){
      ch=arr[i];
      v=0; for(j=1;j<=7;j++){ if(sym[j]==ch){ v=val[j]; break } }
      if(v<prev) total-=v; else { total+=v; prev=v }
    }
    print total"\t"roman"\t"name
  }
' "$INDEX_FILE" 2>/dev/null > "$chapters_tsv" || true

roman_from_int() {
  # Usage: roman_from_int 12
  awk -v num="$1" '
    function out(s){ printf "%s", s }
    BEGIN{
      split("1000 900 500 400 100 90 50 40 10 9 5 4 1", vals, " ")
      split("M CM D CD C XC L XL X IX V IV I", syms, " ")
      n=num+0
      for(i=1;i<=length(vals);i++){
        v=vals[i]+0
        while(n>=v){ n-=v; out(syms[i]) }
      }
    }
  '
}

derive_title_from_filename() {
  local bf="$1"
  local raw
  raw="${bf#*.*.}"
  raw="${raw%.md}"
  raw="${raw//-/ }"
  printf '%s' "$raw" | awk '{ s=tolower($0); printf toupper(substr(s,1,1)) substr(s,2) }'
}

title_for_file() {
  local bf="$1"
  local t
  t=$(awk -F"\t" -v f="$bf" '$1==f{print $2}' "$titles_tsv" | head -n1)
  if [ -z "$t" ]; then
    t=$(derive_title_from_filename "$bf")
  fi
  printf '%s' "$t"
}

chapter_info_for_top() {
  # echo roman<tab>chapter_name for a given top number, or empty
  local top="$1"
  awk -F"\t" -v k="$top" '$1==k{print $2"\t"$3}' "$chapters_tsv" | head -n1
}

strip_existing_nav() {
  # Prints stripped core to stdout
  awk '
    function ltrim(s){ sub(/^\s+/,"",s); return s }
    function rtrim(s){ sub(/\s+$/,"",s); return s }
    function trim(s){ return rtrim(ltrim(s)) }
    {
      lines[++n]=$0
    }
    END{
      i=1
      if(i<=n && trim(lines[i])=="<div align=\"center\">"){
        i++
        while(i<=n && trim(lines[i])=="") i++
        if(i<=n && trim(lines[i])=="#") i++
        if(i<=n && substr(ltrim(lines[i]),1,1)=="[") i++
        while(i<=n && trim(lines[i])=="") i++
        if(i<=n && substr(ltrim(lines[i]),1,3)=="## "){
          i++
          if(i<=n && trim(lines[i])=="") i++
        }
        if(i<=n && substr(ltrim(lines[i]),1,4)=="### "){
          i++
          if(i<=n && trim(lines[i])=="") i++
        }
        # optional submenu template: a link line like [the](...) | [sub](...) | [menu](...)
        if(i<=n && substr(ltrim(lines[i]),1,1)=="["){
          i++
          if(i<=n && trim(lines[i])=="") i++
        }
        if(i<=n && trim(lines[i])=="</div>"){
          i++
          if(i<=n && trim(lines[i])=="") i++
        }
      }
      start=i

      j=n
      while(j>=1 && trim(lines[j])=="") j--
      k=j; last_div=-1
      for(t=0; t<8 && k>=1; t++){
        if(trim(lines[k])=="<div align=\"center\">") { last_div=k; break }
        k--
      }
      end=j
      if(last_div!=-1){
        p=last_div+1
        while(p<=n && trim(lines[p])=="") p++
        if(p<=n && trim(lines[p])=="#") p++
        if(p<=n && substr(ltrim(lines[p]),1,1)=="[") end=last_div-1
      }

      if(start<1) start=1
      if(end<start) { print ""; exit }
      for(x=start; x<=end; x++) print lines[x]
    }
  ' "$1"
}

files=( $(ls -1 "$DOC_DIR"/*.md | sort) )
if [ ${#files[@]} -eq 0 ]; then
  echo "No markdown files found in $DOC_DIR/" >&2
  exit 0
fi

for idx in "${!files[@]}"; do
  path="${files[$idx]}"
  bf="$(basename "$path")"

  if [ "$idx" -eq 0 ]; then
    prev_file="$(basename "${files[0]}")"
    prev_label="END"
  else
    prev_file="$(basename "${files[$((idx-1))]}")"
    prev_label="$(title_for_file "$prev_file")"
  fi

  if [ "$idx" -eq $((${#files[@]} - 1)) ]; then
    next_file="$(basename "${files[$idx]}")"
    next_label="END"
  else
    next_file="$(basename "${files[$((idx+1))]}")"
    next_label="$(title_for_file "$next_file")"
  fi

  cur_title="$(title_for_file "$bf")"

  # Build header
  header_start="<div align=\"center\">\n\n[< *${prev_label}*](${prev_file}) | [**Summary**](0.0.index.md) | [*${next_label}* >](${next_file})\n"

  header_titles="\n## VXLGS Documentation\n\n"
  # parse top and sub from filename like 2.3.server.md
  top=""; sub=""
  if [[ "$bf" =~ ^([0-9]+)\.([0-9]+)\..*\.md$ ]]; then
    top="${BASH_REMATCH[1]}"; sub="${BASH_REMATCH[2]}"
  fi
  roman=""; chapter_name=""
  if [ -n "$top" ]; then
    ci="$(chapter_info_for_top "$top")" || true
    if [ -n "$ci" ]; then
      roman="${ci%%$'\t'*}"
      chapter_name="${ci#*$'\t'}"
    else
      roman="$(roman_from_int "$top")"
    fi
  fi
  if [ -n "$chapter_name" ]; then
    header_titles+="### ${roman}.${sub}. ${chapter_name} - ${cur_title}\n\n"
  elif [ -n "$roman" ] && [ -n "$sub" ]; then
    header_titles+="### ${roman}.${sub}. ${cur_title}\n\n"
  else
    header_titles+="### ${cur_title}\n\n"
  fi

  # Submenu template for non-index pages
  header_submenu=""
  if [ "$bf" != "0.0.index.md" ]; then
    header_submenu+="[the](${bf}#) | [sub](${bf}#) | [menu](${bf}#)\n\n"
  fi

  if [ "$bf" = "0.0.index.md" ]; then
    header_end=""    # Keep header div open on index
  else
    header_end="</div>\n"
  fi

  footer=$'<div align="center">

#
'
  footer+="[< *${prev_label}*](${prev_file}) | [**Summary**](0.0.index.md) | [*${next_label}* >](${next_file})"

  core_content="$(strip_existing_nav "$path")"

  # Assemble new content
  {
    printf '%b' "$header_start"
    printf '%b' "$header_titles"
    printf '%b' "$header_submenu"
    printf '%b' "$header_end"
    if [ -n "${core_content//[$'\n' ]/}" ]; then
      printf '\n%s\n\n' "$core_content"
    else
      printf '\n'
    fi
    printf '%b' "$footer"
  } > "$path.new"

  mv "$path.new" "$path"
  echo "Updated: $path"
done
