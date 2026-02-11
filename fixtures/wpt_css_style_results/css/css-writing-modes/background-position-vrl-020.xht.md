# css/css-writing-modes/background-position-vrl-020.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/background-position-vrl-020.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      background-image: url("support/100x100-red.png");
      background-position: left center;
      background-repeat: repeat-y;
      width: auto; /* Very important: we intentionally want the
      document root element to not fill the viewport width */
      writing-mode: vertical-rl;
    }

  div#reference-overlapping-green
    {
      background-color: green;
      height: 100%;
      position: absolute;
      right: 273px;
      /*
        8px : body's margin-right
    +
      357px : pass-cdts-background-position.png's image width
    +
        8px : body's margin-left
    -
      100px : background-image width (100x100-red.png)
    ==========
      273px is
      */
      top: 0px;
      width: 100px;
    }

    /*
    This test requires a viewport width of at least (strict minimum) of 473px !
    */
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
