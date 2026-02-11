# css/css-anchor-position/registered-custom-property-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/registered-custom-property-anchor.html"
}
```

## style[0]

```css

  @property --length {
    syntax: "<length>";
    inherits: false;
    initial-value: 0px;
  }
  @property --length-percentage {
    syntax: "<length-percentage>";
    inherits: false;
    initial-value: 0px;
  }
  @property --number {
    syntax: "<number>";
    inherits: false;
    initial-value: 0;
  }
  #anchor {
    --length: anchor(--foo bottom, 5px);
    --length-percentage: anchor(--foo bottom, 10%);
    --number: sign(anchor(--foo bottom, 100px));
  }
  #anchor-size {
    --length: anchor-size(--foo block, 7px);
    --length-percentage: anchor-size(--foo block, 20%);
    --number: sign(anchor-size(--foo block, 100px));
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
