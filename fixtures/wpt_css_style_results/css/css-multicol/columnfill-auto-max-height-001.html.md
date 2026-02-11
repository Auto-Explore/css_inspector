# css/css-multicol/columnfill-auto-max-height-001.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/columnfill-auto-max-height-001.html"
}
```

## style[0]

```css

  div
    {
      color: green;
      column-count: 2;
      column-fill: auto; /* fill columns sequentially */
      column-gap: 4em; /* computes to 100px */
      column-rule: red solid 4em;
      font-family: Ahem;
      /*
      To download Ahem font:
      http://www.w3.org/Style/CSS/Test/Fonts/Ahem/
      */
      font-size: 25px;
      line-height: 1;
      max-height: 100px;
      width: 300px;
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
