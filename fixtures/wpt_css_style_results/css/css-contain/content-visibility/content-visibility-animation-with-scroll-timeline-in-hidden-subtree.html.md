# css/css-contain/content-visibility/content-visibility-animation-with-scroll-timeline-in-hidden-subtree.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-animation-with-scroll-timeline-in-hidden-subtree.html"
}
```

## style[0]

```css

#container {
  content-visibility: visible;
}

#scrollContainer {
  height: 100vh;
  overflow-y: scroll;
  scroll-timeline-name: --targetTimeline;
}
#innerspacer {
    height: 300vh;
}
@keyframes fade {
  from { opacity: 1; }
  to { opacity: 0;  }
}
#target {
  background: green;
  height: 100px;
  width: 100px;
}
.animate {
  animation-name: fade;
  animation-duration: 1ms;
  animation-direction: alternate;
  animation-timeline: --targetTimeline;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “scroll-timeline-name”.",
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
