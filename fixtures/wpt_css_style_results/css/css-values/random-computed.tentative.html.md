# css/css-values/random-computed.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-values/random-computed.tentative.html"
}
```

## style[0]

```css

  @property --x {
    syntax: "<number>";
    inherits: true;
    initial-value: 3;
  }
  @property --y {
    syntax: "<number>";
    inherits: true;
    initial-value: 3;
  }
  @property --random-length-1 {
    syntax: "<length>";
    inherits: true;
    initial-value: 3px;
  }
  @property --random-length-2 {
    syntax: "<length>";
    inherits: true;
    initial-value: 3px;
  }
  @property --random-in-initial {
    syntax: "<number>";
    inherits: false;
    initial-value: random(1, 30);
  }
  #container {
    font-size: 30px;
  }
  .randomNoIdentifier {
    width: random(0px, 100px);
    height: random(0px, 100px);
    left: random(0px, 100000px);
    right: random(0px, 100000px);
    margin: random(0px, 100000px) random(0px, 100000px);
    --x: random(0, 100);
    --y: random(0, 100);
    --random-length-1: random(fixed random(0, 1), 10px, 100px);
    --random-length-2: random(fixed random(0, 1), 10px, 100px);
  }
  .randomMatchElement {
    width: random(element-shared, 0px, 100px);
    height: random(element-shared, 0px, 100px);
    left: random(element-shared, 0px, 100000px);
    right: random(element-shared, 0px, 100000px);
    margin: random(element-shared 0px, 100000px) random(element-shared 0px, 100000px);
    translate: random(element-shared, 10%, 30%);
    scale: random(element-shared, 1, 3) random(element-shared, 3, 9);
  }
  .randomIdentifier {
    width: random(--identifier, 0px, 100px);
    height: random(--identifier, 0px, 100px);
    left: random(--identifier, 0px, 100000px);
    right: random(--identifier, 0px, 100000px);
    margin: random(--identifier 0px, 100000px) random(--identifier 0px, 100000px);
  }
  .randomMatchElementAndIdentifier {
    width: random(element-shared --other-identifier, 0px, 100px);
    height: random(element-shared --other-identifier, 0px, 100px);
    left: random(element-shared --other-identifier, 0px, 100000px);
    right: random(element-shared --other-identifier, 0px, 100000px);
    margin: random(element-shared --other-identifier 0px, 100000px) random(element-shared --other-identifier 0px, 100000px);
  }
  .randomFixed {
    width: random(fixed 0.5, 10px, 100px);
    height: random(fixed 0.5, 10px, 100px);
    left: random(fixed 0.5, 0px, 100000px);
    right: random(fixed 0.5, 0px, 100000px);
    margin: random(fixed 0.5 0px, 100000px) random(fixed 0.5 0px, 100000px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
