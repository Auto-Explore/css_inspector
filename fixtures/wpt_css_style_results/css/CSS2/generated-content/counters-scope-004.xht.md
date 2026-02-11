# css/CSS2/generated-content/counters-scope-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-scope-004.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  .reset { counter-reset: c; }
  .rb:before { counter-reset: c; content: "R"; }
  .use { counter-increment: c; }
  .use:before { content: counters(c, ".") " "; }

  
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
