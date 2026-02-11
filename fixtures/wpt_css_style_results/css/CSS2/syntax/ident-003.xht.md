# css/CSS2/syntax/ident-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/ident-003.xht"
}
```

## style[0]

```css


  .one { color: red; background: white; }
  .-ident, .one { color: green; }

  .two { color: red; background: white; }
  #-ident, .two { color: green; }

  .three { color: green; background: white; }
  .-1ident, .three { color: red; }

  .four { color: green; background: white; }
  #-1ident, .four { color: red; }

  .five { color: red; background: white; }
  .-\31ident, .five { color: green; }

  .six { color: red; background: white; }
  #-\31ident, .six { color: green; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
