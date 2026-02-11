# css/css-anchor-position/inline-grid-try-fallbacks-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/inline-grid-try-fallbacks-crash.html"
}
```

## style[0]

```css

    @keyframes --anim {
      from {
        position: absolute;
        position-try-fallbacks: flip-block;
      }
    }
    #parent {
      display: inline-grid;
    }
    #victim {
      height:100em;
    }
    #victim.anim {
      animation: --anim 100ms linear;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
