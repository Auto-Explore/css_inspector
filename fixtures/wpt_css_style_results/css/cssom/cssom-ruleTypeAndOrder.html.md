# css/cssom/cssom-ruleTypeAndOrder.html

```json
{
  "format_version": 3,
  "file": "css/cssom/cssom-ruleTypeAndOrder.html"
}
```

## style[0]

```css

    @namespace svg "http://www.w3.org/2000/svg";
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    @import url('main.css');
  
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```

## style[2]

```css

    h1 { background: indianred; }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

    @namespace svg "http://www.w3.org/2000/svg";
    svg|a { color: white; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

    @font-face {
      font-family: 'Megalopolis';
      src: url('fonts/megalopolisextra-webfont.eot');
      src: url('fonts/megalopolisextra-webfont.eot?#iefix') format('embedded-opentype'),
           url('fonts/megalopolisextra-webfont.woff') format('woff'),
           url('fonts/megalopolisextra-webfont.ttf') format('truetype'),
           url('fonts/megalopolisextra-webfont.svg#MEgalopolisExtraRegular') format('svg');
      font-weight: normal;
      font-style: normal;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

    @media (min-width: 200px) {
        h1 { background: aliceblue; }
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

    @page :first {
        h1 { color: #444; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
