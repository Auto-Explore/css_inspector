# css/CSS2/generated-content/content-counters-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-016.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test { counter-reset: c; }
  p, #test span { counter-increment: c; }

  span#one:before { content: counters(c, ".") " - "; }
  span#two:before { content: counters(c, "") " - "; }
  span#three:before { content: counters(c, ".", decimal) "--    --" counters(c, "-------", decimal); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
