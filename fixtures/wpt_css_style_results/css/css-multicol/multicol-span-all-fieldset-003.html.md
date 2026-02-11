# css/css-multicol/multicol-span-all-fieldset-003.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-fieldset-003.html"
}
```

## style[0]

```css

  fieldset {
    column-count: 3;
    column-rule: 6px solid;
    width: 400px;
    overflow: hidden;
    position: absolute;
  }
  h3 {
    column-span: all;
    outline: 1px solid blue;
  }
  a {
    position: fixed;
    width: 1em;
    height: 1em;
    background-color: blue;
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
