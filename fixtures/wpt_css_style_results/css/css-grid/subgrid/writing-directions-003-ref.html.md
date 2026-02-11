# css/css-grid/subgrid/writing-directions-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/writing-directions-003-ref.html"
}
```

## style[0]

```css


.grid {
  display: grid;
  grid-template: repeat(5, 10px) repeat(5, 10px) / repeat(5, 10px) repeat(5, 10px);
  background: grey;
  border: 1px solid;
  width: 100px;
  height: 100px;
}

.grid-item {
  width: 10px;
  height: 10px;
  background: orange;
  border: 1px solid;
}

.rtl { direction:rtl; }

.ltr { direction:ltr; }

  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
