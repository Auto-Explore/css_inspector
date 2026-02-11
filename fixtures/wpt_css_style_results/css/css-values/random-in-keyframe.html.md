# css/css-values/random-in-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-values/random-in-keyframe.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
        translate: 0px;
        translate: random(2px, 200px);
    }
    to {
        translate: 0px;
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
