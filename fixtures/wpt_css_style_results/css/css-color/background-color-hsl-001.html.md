# css/css-color/background-color-hsl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-color/background-color-hsl-001.html"
}
```

## style[0]

```css

      #p1 { background-color: hsla(120.0, 75%, 50%, 20%); }
      #p2 { background-color: hsla(120, 75%, 50%, 0.4); }
      #p3 { background-color: hsla(120 75% 50% / 60%); }
      #p4 { background-color: hsla(120.0 75% 50% / 1.0); }
      #p5 { background-color: hsla(120/* comment */75%/* comment */50%/1.0); }
      #p6 { background-color: hsla(120,/* comment */75%,/* comment */50%,100%); }
      #p7 { background-color: hsla(120.0, 75%, 50%); }
      #p8 { background-color: hsla(120 75% 50%); }
      #p9 { background-color: hsla(120/* comment */75%/* comment */50%); }
      #p10 { background-color: hsla(120/* comment */,75%,/* comment */50%); }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
