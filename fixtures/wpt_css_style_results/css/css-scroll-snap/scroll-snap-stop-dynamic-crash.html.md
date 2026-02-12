# css/css-scroll-snap/scroll-snap-stop-dynamic-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-stop-dynamic-crash.html"
}
```

## style[0]

```css

div { overflow-y: scroll; }
::-webkit-scrollbar { width: 10px; }
::-webkit-scrollbar-corner { }
.crash::-webkit-scrollbar-corner {
  scroll-snap-stop: always;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
