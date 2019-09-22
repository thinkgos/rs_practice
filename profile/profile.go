package profile

import (
	"net/url"
	"strconv"
)

const (
	/**
	 * 连接认证类型
	 */
	AUTH_TYPE = "name"

	/**
	 * 使用AppKey连接
	 */
	AUTH_TYPE_APP_KEY = "appkey"

	/**
	 * 使用阿里云AK连接
	 */
	AUTH_TYPE_ACCESS_KEY = "aliyunak"

	/**
	 * 使用productKey和deviceName连接
	 */
	AUTH_TYPE_DEVICE_NAME = "devicename"

	/**
	 * 使用AppKey方式连接，参数AppKey
	 */
	PARAM_APP_KEY = "param-app-key"

	/**
	 * 使用AppKey方式连接，参数AppSecret
	 */
	PARAM_APP_SECRET = "appSecret"

	/**
	 * 使用设备方式连接，productKey
	 */
	PARAM_PRODUCT_KEY = "param-product-key"

	/**
	 * 使用设备方式连接，deviceName
	 */
	PARAM_DEVICE_NAME = "param-device-name"

	/**
	 * 使用设备方式连接，deviceSecret
	 */
	PARAM_DEVICE_SECRET = "deviceSecret"

	/**
	 * 使用设备方式连接，clientId
	 */
	PARAM_CLIENT_ID = "param-client-id"

	/**
	 * 使用阿里云AK方式连接，region
	 */
	REGION_ID = "regionId"

	/**
	 * 使用阿里云AK方式连接，accessKey
	 */
	PARAM_ACCESS_KEY = "accessKey"

	/**
	 * 使用阿里云AK方式连接，accessSecret
	 */
	PARAM_ACCESS_SECRET = "accessSecret"

	/**
	 * 使用阿里云AK方式连接，accessKey
	 */
	PARAM_STS_TOKEN = "stsToken"

	/**
	 * 使用阿里云POP接口HOST
	 */
	PARAM_ALIYUNCS_HOST = "aliyuncs-host"

	/**
	 * 连接认证类型
	 */
	INSTANCE_ID = "instanceId"
)

const (
	EnableSSL   = true
	DefaultPort = 443
)

type Profile struct {
	EndPoint          string
	AuthParams        map[string]string
	MultiConnection   bool
	Port              int
	CleanSession      bool
	HeartBeatInterval int
	HeartBeatTimeOut  int
}

func newProfile(endPoint string) *Profile {
	return &Profile{
		EndPoint:          endPoint,
		AuthParams:        make(map[string]string),
		HeartBeatInterval: 30 * 1000,
		HeartBeatTimeOut:  60 * 1000,
	}
}

func NewDeviceProfile(endPoint, productKey, deviceName, deviceSecret, clientId string) *Profile {
	p := newProfile(endPoint)
	p.AuthParams[AUTH_TYPE] = AUTH_TYPE_DEVICE_NAME
	p.AuthParams[PARAM_PRODUCT_KEY] = productKey
	p.AuthParams[PARAM_DEVICE_NAME] = deviceName
	p.AuthParams[PARAM_DEVICE_SECRET] = deviceSecret
	p.AuthParams[PARAM_CLIENT_ID] = clientId
	p.MultiConnection = false
	return p
}

func NewAppKeyProfile(endPoint, appKey, appSecret string) *Profile {
	p := newProfile(endPoint)
	p.AuthParams[AUTH_TYPE] = AUTH_TYPE_APP_KEY
	p.AuthParams[PARAM_APP_KEY] = appKey
	p.AuthParams[PARAM_APP_SECRET] = appSecret
	p.MultiConnection = true
	return p
}

func NewAccessKeyProfileToken(endPoint, regionId, accessKey, accessSecret, stsToken string) *Profile {
	p := newProfile(endPoint)
	p.AuthParams[AUTH_TYPE] = AUTH_TYPE_ACCESS_KEY
	p.AuthParams[PARAM_ACCESS_KEY] = accessKey
	p.AuthParams[PARAM_ACCESS_SECRET] = accessSecret
	p.AuthParams[REGION_ID] = regionId
	p.AuthParams[PARAM_STS_TOKEN] = stsToken
	p.MultiConnection = true
	return p
}

func NewAccessKeyProfile(endPoint, regionId, accessKey, accessSecret string) *Profile {
	return NewAccessKeyProfileToken(endPoint, regionId, accessKey, accessSecret, nil)
}

// HostPort  获得hostname 和 port
// 注意hostname可能为空字符串,port获取失败为默认端口443
func (this *Profile) HostPort() (string, int) {
	ur, err := url.Parse(this.EndPoint)
	if err != nil {
		return "", DefaultPort
	}

	port, err := strconv.ParseInt(ur.Port(), 10, 0)
	if err != nil {
		return ur.Hostname(), DefaultPort
	}
	return ur.Hostname(), int(port)
}
