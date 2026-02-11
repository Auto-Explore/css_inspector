# css/css-position/sticky/position-sticky-bottom-002.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-bottom-002.html"
}
```

## style[0]

```css

  div.scrolling-container
    {
      background-image: url("support/100x100-red.png");
      background-repeat: no-repeat;
      display: inline-block;
      height: 250px;
      margin-right: 30px;
      overflow: auto;
      position: static;
      width: 150px;
    }

  div#first-scrolling-container
    {
      background-position: left 75px;
    }

  div#second-scrolling-container
    {
      background-position: left 50px;
    }

  div#third-scrolling-container
    {
      background-position: left top;
    }

  div.vertical-spacer
    {
      height: 100px;
    }

  div.sticky
    {
      background-color: green;
      bottom: 100px;
      height: 100px;
      position: sticky;
      width: 100px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
