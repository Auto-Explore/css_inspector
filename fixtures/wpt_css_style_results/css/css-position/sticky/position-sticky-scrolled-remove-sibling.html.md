# css/css-position/sticky/position-sticky-scrolled-remove-sibling.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-scrolled-remove-sibling.html"
}
```

## style[0]

```css

  div#scrollingContainerVert
    {
      background-color: red;
      height: 100px;
      overflow: auto;
      width: 200px;
    }

  div#scrollingContainerHoriz
    {
      background-color: red;
      height: 100px;
      overflow-x: auto;
      overflow-y: hidden;
      width: 200px;
    }

  div#elemStickyVert
    {
      background-color: green;
      height: 100px;
      position: sticky;
      top: 0px;
    }

  div#scrollingContainerHoriz > div
    {
      display: inline-block;
      height: 100%;
    }

  div#elemStickyHoriz
    {
      background-color: green;
      left: 0px;
      position: sticky;
      width: 200px;
    }

  div#tallItem
    {
      height: 600px;
    }

  div#wideItem
    {
      width: 600px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
