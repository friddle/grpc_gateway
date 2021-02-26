error->use httpbis(需要处理)


然后把常见的Http2错误做一个Status
Http(httpbis::error)->Http(ErrorStatus)
提供一个From 

