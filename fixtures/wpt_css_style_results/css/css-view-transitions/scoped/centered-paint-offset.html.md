# css/css-view-transitions/scoped/centered-paint-offset.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/centered-paint-offset.html"
}
```

## style[0]

```css


body { margin: 20px }
#scope { contain: layout; text-align: center;
  width: 200px; height: 100px; padding: 20px;
  background: #eee; border: 20px solid #acf; }
#scope::view-transition-group(*) { animation-play-state: paused; }
#scope::view-transition-new(*) { animation: unset; opacity: 1; }
#scope::view-transition-old(*) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
