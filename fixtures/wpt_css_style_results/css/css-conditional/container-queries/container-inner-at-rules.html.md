# css/css-conditional/container-queries/container-inner-at-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-inner-at-rules.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 100px;
    height: 100px;
  }

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

  @container (width: 100px) {
    @keyframes anim1 {
      from { --anim1:true; }
      to { --anim1:true; }
    }
  }

  @container (width: 200px) {
    @keyframes anim2 {
      from { --anim2:true; }
      to { --anim2:true; }
    }
  }

  #child { animation: anim1 10s paused, anim2 10s paused; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  @container (width: 100px) {
    @property --prop1 {
      syntax: "<length>";
      inherits: false;
      initial-value: 0px;
    }
  }

  @container (width: 200px) {
    @property --prop2 {
      syntax: "<length>";
      inherits: false;
      initial-value: 0px;
    }
  }

  #child {
    font-size: 20px;
    --prop1:1em;
    --prop2:2em;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

  @container (width: 100px) {
    @layer a;
  }

  @container (width: 200px) {
    @layer b;
  }

  @layer b {
    #child { --layer:b; }
  }

  @layer a {
    #child { --layer:a; }
  }

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

  @container (width: 100px) {
    @font-face {
      font-family: Font1;
      font-stretch: 50% 200%;
      src: url(/fonts/Ahem.ttf);
    }
  }

  @container (width: 200px) {
    @font-face {
      font-family: Font2;
      font-stretch: 40% 190%;
      src: url(/fonts/Ahem.ttf);
    }
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

  @container (width: 100px) {
    /* Assumed to be false */
    @media (width: 0px) {
      #child { --media1:true; }
    }
    /* Assumed to be true */
    @media (min-width: 0px) {
      #child { --media2:true; }
    }
  }

  /* Same again, but with failing container query. */
  @container (width: 200px) {
    @media (width: 0px) {
      #child { --media3:true; }
    }
    @media (min-width: 0px) {
      #child { --media4:true; }
    }
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

  @container (width: 100px) {
    @supports (width: 500kg) {
      #child { --supports1:true; }
    }
    @supports (width: 500px) {
      #child { --supports2:true; }
    }
  }

  /* Same again, but with failing container query. */
  @container (width: 200px) {
    @supports (width: 500kg) {
      #child { --supports3:true; }
    }
    @supports (width: 500px) {
      #child { --supports4:true; }
    }
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
