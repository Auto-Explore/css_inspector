# css/css-conditional/container-queries/container-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-nested.html"
}
```

## style[0]

```css


#outer {
  container-name: outer;
  container-type: size;
  width: 100px;
  height: 100px;
}

#inner {
  container-name: inner;
  container-type: size;
  width: 50px;
  height: 50px;
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

  @container (width: 50px) {
    @container (height: 50px) {
      #child { --implicit:true; }
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

## style[2]

```css

  @container (width: 70px) {
    @container (height: 50px) {
      #child { --implicit-outer-fail:true; }
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

## style[3]

```css

  @container (width: 50px) {
    @container (height: 70px) {
      #child { --implicit-inner-fail:true; }
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

## style[4]

```css

  @container outer (width: 100px) {
    @container inner (height: 50px) {
      #child { --named-outer-inner:true; }
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

  @container inner (width: 50px) {
    @container outer (height: 100px) {
      #child { --named-outer-inner-reverse:true; }
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

  @container unknown (width: 100px) {
    @container inner (height: 50px) {
      #child { --named-failing-outer:true; }
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

## style[7]

```css

  @container outer (width: 100px) {
    @container unknown (height: 50px) {
      #child { --named-failing-inner:true; }
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

## style[8]

```css

  @container outer (width: 100px) {
    @container (height: 50px) {
      #child { --named-outer-inner-implicit:true; }
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

## style[9]

```css

  @container (width: 50px) {
    @container inner (height: 50px) {
      #child { --implicit-outer-inner-named:true; }
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

## style[10]

```css

  @container (width: 50px) {
    @container outer (height: 100px) {
      #child { --implicit-outer-inner-named-reverse:true; }
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

## style[11]

```css

  @container (width > 1px) {
    @container (width > 2px) {
      @container (width > 3px) {
        #child { --three-levels:true; }
      }
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

## style[12]

```css

  @container (width > 1px) {
    @container (width > 2000px) {
      @container (width > 3px) {
        #child { --three-levels-middle-fail:true; }
      }
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

## style[13]

```css

  @container (width: 50px) {
    @container outer (height: 100px) {
      #child { --inner-named-invalidation:true; }
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

## style[14]

```css

  @container (width: 50px) {
    @container outer (height: 100px) {
      #child { --outer-implicit-invalidation:true; }
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
