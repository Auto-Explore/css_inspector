# css/CSS2/syntax/malformed-decl-block-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/malformed-decl-block-001.xht"
}
```

## style[0]

```css

      body {background: white; color: red}
      #p1 {color: green}
      #p2 {@charset utf-8; color: green}
      #p3 {@foo {color: red} color: green}
      #p4 {12; color: green}
      #p5 {color: green; 12 color: red}
      #p6 {color: orange; 12 @page {color: red} color: green}
      #p7 {@foo {color: red}; color: green}
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
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
