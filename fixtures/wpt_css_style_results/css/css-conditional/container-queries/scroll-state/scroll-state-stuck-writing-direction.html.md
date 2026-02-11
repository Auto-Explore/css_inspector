# css/css-conditional/container-queries/scroll-state/scroll-state-stuck-writing-direction.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-stuck-writing-direction.html"
}
```

## style[0]

```css

  #filler {
    height: 10000px;
  }
  #stuck {
    container-type: scroll-state;
    position: sticky;
    bottom: 0;
    width: 100px;
    height: 100px;
    background: lime;
  }
  #target {
    writing-mode: horizontal-tb;
    direction: ltr;
    width: 100px;
    height: 100px;
    background: orange;
  }
  @container scroll-state(stuck: inline-start) {
    #target { --stuck: inline-start }
  }
  @container scroll-state(stuck: inline-end) {
    #target { --stuck: inline-end }
  }
  @container scroll-state(stuck: block-start) {
    #target { --stuck: block-start }
  }
  @container scroll-state(stuck: block-end) {
    #target { --stuck: block-end }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
