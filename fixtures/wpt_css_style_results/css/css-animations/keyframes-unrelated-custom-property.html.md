# css/css-animations/keyframes-unrelated-custom-property.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/keyframes-unrelated-custom-property.html"
}
```

## style[0]

```css

  @keyframes test {
    0% { --x: green; }
    100% { --x: green; }

    /* This should not affect the background-color of #div: */
    0% { --unused: yellow; }
  }
  #div {
    animation: test 10s linear paused;
    background-color: var(--x, red);
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
