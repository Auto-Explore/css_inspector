# css/CSS2/colors/colors-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/colors/colors-007.xht"
}
```

## style[0]

```css

    p.correct { color: red }
    p.incorrect { color: green }
    p#keyword { color: green }
    p#quoted { color: 'red'; color: "red"; }
    p#hash { color: #red }
    p#escape { color: g\re\45n }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
