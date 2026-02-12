# css/css-pseudo/highlight-painting-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-001-ref.html"
}
```

## style[0]

```css

    p {
        font-size: 7em;
        text-shadow: 0.2500em 0.2500em #C0C000;
        position: relative;
        color: transparent;
    }
    p > span {
        color: initial;
        text-shadow: none;
        position: absolute;
    }
    p > span > span {
        background: #C0C0C0;
        text-shadow: 0.5000em 0.5000em #3838E0;
        /* force blue to paint over red */
        position: relative;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
