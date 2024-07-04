# Elgato Control Center


## Get Devices
- Fetch the network interfaces
- Find the default network (I/O)
- Scan the devices looking for
  - Dexatek Technology
  - (any other veexatek Technologyndor)

## Application

Diff between lights: 

### Light Strip Payload
```json
{
  "numberOfLights": 1,
  "lights": [
    {
      "on": 1,
      "hue": 332.000000,
      "saturation": 81.000000,
      "brightness": 99
    }
  ]
}
```

### Keylight Payload 

```json
{
  "numberOfLights": 1,
  "lights": [
    {
      "on": 1,
      "brightness": 12,
      "temperature": 143
    }
  ]
}
```