# css/css-inline/text-box-trim/text-box-trim-not-ignore-nested-text-box-edge.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-not-ignore-nested-text-box-edge.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 100px;
}
.target {
  font: 50px/2 Ahem;
  text-box-trim: trim-both;
  text-box-edge: ex alphabetic;
}
.inner {
  text-box-edge: auto;
  /* auto keyword uses the value of line-fit-edge on the root inline box of the affected line box,
  interpreting leading (the initial value) as text. */
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-trim”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
