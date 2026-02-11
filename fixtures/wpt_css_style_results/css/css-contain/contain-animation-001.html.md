# css/css-contain/contain-animation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-animation-001.html"
}
```

## style[0]

```css

div {
  border: 50px solid green;
  background: red;
  position: absolute; /* for shrinkwrap */
  contain: strict;

  animation-duration: 1s;
  animation-name: bad;
  animation-play-state: paused;

  font-size: 100px;
}

@keyframes bad {
  from {
    contain: none;
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
