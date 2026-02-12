# css/css-anchor-position/position-try-backdrop.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-backdrop.html"
}
```

## style[0]

```css

  @position-try --pt {
    left: 300px;
  }
  #anchor {
    width: 100px;
    height: 100px;
    margin-left: 150px;
    margin-top: 50px;
    background: coral;
  }
  dialog::backdrop {
    background: seagreen;
    width:100px;
    height:100px;
    left: 9999px; /* Force overflow */
    position-try-fallbacks: --pt;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
