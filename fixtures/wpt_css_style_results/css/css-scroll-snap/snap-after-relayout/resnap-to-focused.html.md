# css/css-scroll-snap/snap-after-relayout/resnap-to-focused.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/resnap-to-focused.html"
}
```

## style[0]

```css


#snapper {
    counter-reset: child 0;
    width: 200px;
    scroll-snap-type: block mandatory;
    overflow:hidden;
    height: 100px;
}
.child {
    width: 100px;
    height: 100px;
    background:red;
    text-align: center;
    box-sizing: border-box;
    counter-increment: child;
    float: left;
}
.child.f {
    background: green;
    scroll-snap-align: center;
}
.child::before {
    content: counter(child);
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
