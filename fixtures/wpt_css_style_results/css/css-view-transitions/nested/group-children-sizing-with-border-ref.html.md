# css/css-view-transitions/nested/group-children-sizing-with-border-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/group-children-sizing-with-border-ref.html"
}
```

## style[0]

```css

#wrapper {
  position: relative;
}

#clipper {
  height: 200px;
  width: 200px;
  overflow: clip;

  border-width: 5px 10px 15px 20px;
  border-style: solid;
  border-color: green;
}

.item {
  background: blue;
  position: relative;
  top: -25px;
  left: -10px;

  height: 50px;
  width: 250px;
  margin: 1px;
  border: 1px solid black;
}

.popout {
  position: absolute;
  left: 11px;
  top: 87px;
  background: blue;

  height: 50px;
  width: 250px;
  border: 1px solid black;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
