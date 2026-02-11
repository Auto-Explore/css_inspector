# css/css-conditional/container-queries/scroll-state/scroll-state-initially-stuck.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-initially-stuck.html"
}
```

## style[0]

```css

  #filler {
    height: 10000px;
  }
  #stuck {
    container-name: initially-stuck;
    container-type: scroll-state;
    position: sticky;
    bottom: 0;
  }

  @container initially-stuck scroll-state(stuck: bottom) {
    span { --stuck: yes }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
