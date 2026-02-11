# css/CSS2/syntax/escapes-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-004.xht"
}
```

## style[0]

```css


  p.class { background: red; color: white; }
  p.c\00006Cas\000073 { back\000067round: gr\000065en; }

  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
