# css/css-text/white-space/text-wrap-balance-line-clamp-005.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-balance-line-clamp-005.html"
}
```

## style[0]

```css

body > div {
    border: solid;
    font-family: monospace;
    margin: 1ch;
    width: 11.1ch;  /* .1ch to work around browser bugs */
}
.test {
    border-color: blue;

    line-clamp: 3;
    /* This code is unnecessary in any browser that supports the unprefixed version of line-clamp,
       but neither does it have any detrimental effect,
       and it broadens the test to browsers that only support the prefixed version */

    -webkit-line-clamp: 3;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
.balanced {
    text-wrap-style: balance;
}
.ref {
    border-color: orange;
    white-space: nowrap;
}
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
