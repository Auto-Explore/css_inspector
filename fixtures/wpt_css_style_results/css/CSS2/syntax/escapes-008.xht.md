# css/CSS2/syntax/escapes-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-008.xht"
}
```

## style[0]

```css


  p.class { background: green; color: white; }
  p.c\06C  ass { back\67round: r\000065 ed; }
  p.c\06Cass { back\67
 round: r\000065ed; }
  p.c\06Cass { back\67round: r\000065
ed; }

  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
