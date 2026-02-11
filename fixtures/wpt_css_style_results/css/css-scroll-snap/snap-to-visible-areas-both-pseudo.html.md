# css/css-scroll-snap/snap-to-visible-areas-both-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-to-visible-areas-both-pseudo.html"
}
```

## style[0]

```css


body, html { height: 100%; }

div {
  position: absolute;
  margin: 0px;
}

#scroller {
  height: 600px;
  width: 600px;
  overflow: scroll;
  scroll-snap-type: both mandatory;
}

#space {
  width: 2000px;
  height: 2000px;
}

.snap {
  width: 200px;
  height: 200px;
  background-color: blue;
  scroll-snap-align: start;
}

#left-top {
  left: 0px;
  top: 0px;
}

#left-top::before {
  position: absolute;
  margin: 0px;
  content: "";

  display:block;

  left: 0px;
  top: 800px;
  width: 200px;
  height: 200px;
  background-color: yellow;
  scroll-snap-align: start;
}

#left-top::after {
  position: absolute;
  margin: 0px;
  content: "";

  display:block;

  left: 800px;
  top: 0px;
  width: 200px;
  height: 200px;
  background-color: yellow;
  scroll-snap-align: start;

}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
