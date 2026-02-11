# css/css-text/hyphens/hyphenate-character-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphenate-character-002.html"
}
```

## style[0]

```css

div {
  font: 16px monospace;
  width: 4.5ch;  /* wide enough that the first potential break in "re-al-iza-tion" should NOT be used */
  hyphens: auto;  /* assuming the usual en_US patterns, should generate the same breaks as the manual
                     soft hyphens in test 001. */
  hyphenate-character: "";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
