# css/css-pseudo/marker-text-align-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-align-001.html"
}
```

## style[0]

```css

li {
  /* Should not be inherited by ::marker */
  text-align: start;
  text-align-all: start;
  text-align-last: start;
}
::marker {
  /* Should have no effect */
  text-align: start;
  text-align-all: start;
  text-align-last: start;
}
li > div {
  text-align: initial;
  text-align-all: initial;
  text-align-last: initial;
}
ol {
  padding-left: 13ch;
}
li {
  line-height: 16px;
  height: 32px;
  white-space: pre;
}
.disc {
  list-style-type: disc;
}
.decimal {
  list-style-type: decimal;
}
.string {
  list-style-type: "[m]\a longtext";
}
.content::marker {
  content: "[m]\a longtext";
}
.rtl-marker ::marker {
  direction: rtl;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
