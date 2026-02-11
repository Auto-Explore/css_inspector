# css/css-animations/animation-play-state-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-play-state-001-manual.html"
}
```

## style[0]

```css

  #test-animation-paused {
    animation-name: sample;
    animation-duration: 10s;

    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
  }

  #ref-path {
    background-color: yellow;
    height: 10px;
    width: 250px;
    position: relative;
  }

  @keyframes sample {
    from {
      left: 150px;
    }
    to {
      left: 0px;
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
