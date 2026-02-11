# css/CSS2/generated-content/counters-scope-implied-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-scope-implied-001.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  .i { counter-increment: c 1; }
  .r { counter-reset: c 0; }
  .u:before { content: counters(c, ".") " "; }

  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
