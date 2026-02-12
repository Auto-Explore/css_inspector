# css/css-anchor-position/at-position-try-invalidation-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/at-position-try-invalidation-shadow-dom.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #host { width: 200px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      ::slotted(#slotted), :host {
        position-try-fallbacks: --pf;
        position: absolute;
        left: 999999px; /* force fallback */
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
