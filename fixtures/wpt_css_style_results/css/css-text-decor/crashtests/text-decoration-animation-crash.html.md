# css/css-text-decor/crashtests/text-decoration-animation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/crashtests/text-decoration-animation-crash.html"
}
```

## style[0]

```css

* {
  text-decoration-line: underline blink;
  -webkit-appearance: progress-bar;
  columns: 11px;
}

#x4 {
  -webkit-user-modify: read-write;
  animation: keyframes4 2s;
}
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

@keyframes keyframes0 {
  20% {
    scroll-padding-left: 1em
  }
}

@keyframes keyframes3 {
  20% {
    scroll-padding-block-end: auto
  }
}

@keyframes keyframes4 {
  60% {
    background-position: 65%
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
