# css/css-text/white-space/text-wrap-pretty-line-clamp-crash-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-pretty-line-clamp-crash-001.html"
}
```

## style[0]

```css

div {
    border: solid;
    font-family: monospace;
    width: 9.1ch;  /* .1ch to work around browser bugs */
}
.test {
    text-wrap-style: pretty;
    line-clamp: 1;
    /* This code is unnecessary in any browser that supports the unprefixed version of line-clamp,
       but neither does it have any detrimental effect,
       and it broadens the test to browsers that only support the prefixed version */

    -webkit-line-clamp: 1;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
