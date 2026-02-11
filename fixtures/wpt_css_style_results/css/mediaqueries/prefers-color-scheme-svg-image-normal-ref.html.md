# css/mediaqueries/prefers-color-scheme-svg-image-normal-ref.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/prefers-color-scheme-svg-image-normal-ref.html"
}
```

## style[0]

```css

  .background {
    width: 32px;
    height: 32px;
  }
  #light { display: revert }
  #dark { display: none }
  @media (prefers-color-scheme: dark) {
    #light { display: none }
    #dark { display: revert }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
