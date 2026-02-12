# css/css-contain/content-visibility/content-visibility-animation-with-scroll-timeline-in-auto-subtree.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-animation-with-scroll-timeline-in-auto-subtree.html"
}
```

## style[0]

```css

#container {
  content-visibility: auto;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
