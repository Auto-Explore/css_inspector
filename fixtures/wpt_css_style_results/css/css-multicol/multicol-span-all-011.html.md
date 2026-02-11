# css/css-multicol/multicol-span-all-011.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-011.html"
}
```

## style[0]

```css

  article {
    column-count: 2;
    column-rule: 6px solid;
    width: 400px;
    outline: 1px solid black;
    unicode-bidi: bidi-override; /* Needed to trigger bidi resolution. */
  }
  h3 {
    column-span: all;
    outline: 1px solid blue;
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
