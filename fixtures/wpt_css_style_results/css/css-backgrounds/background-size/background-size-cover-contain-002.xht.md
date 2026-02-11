# css/css-backgrounds/background-size/background-size-cover-contain-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-size/background-size-cover-contain-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-image: url("support/200x200-red.png");
  background-repeat: no-repeat;
  background-size: contain;
  height: 50px;
  padding: 25px;
  width: 150px;
  /*
  background positioning area is 200px wide by 100px tall.
  The largest size that can fit inside it with a 1:1 ratio
  is 100px by 100px. So, the background image should be
  scaled to 100px by 100px.
  */
  }

  div#overlapping-green
  {
  background-image: url("support/50x50-green.png");
  background-repeat: no-repeat;
  background-size: cover;
  border-right: transparent solid 50px;
  bottom: 100px;
  height: 50px;
  padding: 25px;
  position: relative;
  width: 0px;

  /*
  background positioning area to cover is 50px wide by
  100px tall. The smallest size that can completely cover
  it with a 1:1 ratio is 100px by 100px. So, the
  background image should be scaled to 100px by 100px.
  */
  }
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
