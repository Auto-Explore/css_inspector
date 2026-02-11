# css/css-lists/counter-set-001.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-set-001.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}
span::before { content: counters(n, '.'); }
  
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
