# css/css-pseudo/crashtests/pseudoelement-removal-with-scroll-timeline-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/crashtests/pseudoelement-removal-with-scroll-timeline-crash.html"
}
```

## style[0]

```css

  #target::after {
    content: "content";
    animation: fade-scroll linear;
    animation-timeline: scroll(inline);
  }

  #target.off::after {
    content: none;
  }

  @keyframes fade-scroll {
    from { opacity: 1; }
    to { opacity: 0.3; }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “animation-timeline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
