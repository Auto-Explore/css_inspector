# css/css-view-transitions/view-transition-types-match-early.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-types-match-early.html"
}
```

## style[0]

```css


html:active-view-transition-type(type-name) #target {
  background: green;
  view-transition-name: target;
}

#target {
  background: red;
  width: 100px;
  height: 100px;
}

html::view-transition-group(target) {
  animation-play-state: paused;
}

html::view-transition-new(target) {
  animation: unset;
  opacity: 0;
}

html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}

html::view-transition-group(root) {
  display: none;
}

html::view-transition { background: lightpink; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
