# css/css-multicol/broken-column-rule-1.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/broken-column-rule-1.html"
}
```

## style[0]

```css


.outer {
  height: 100px;
  column-fill: auto;
  width: 550px;
  column-count: 4;
  column-gap: 50px;
  /* leaves 100px for each column */
}

.inner {
  column-count: 2;
  column-rule: 2px solid blue;
  height: 250px;
}

.fill {
  height: 500px; /* work around https://bugzilla.mozilla.org/show_bug.cgi?id=1374479#c4 */
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
