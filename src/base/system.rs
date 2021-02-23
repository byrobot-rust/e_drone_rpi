pub enum ModelNumber {
    None = 0,

    Drone3DroneP1 = 0x00031001,
    Drone3DroneP10 = 0x0003100A,

    Drone4DroneP4 = 0x00041004,
    Drone4DroneP5 = 0x00041005,
    Drone4DroneP6 = 0x00041006,
    Drone4DroneP7 = 0x00041007,

    Drone4ControllerP2 = 0x00042002,
    Drone4ControllerP3 = 0x00042003,
    Drone4ControllerP4 = 0x00042004,

    Drone8DroneP1 = 0x00081004,

    Drone9DroneP2 = 0x00091002,
}

pub enum DeviceType {
    None = 0x00,

    Drone = 0x10, // 드론(Server)

    Controller = 0x20, // 조종기(Client)

    LinkClient = 0x30, // 링크 모듈(Client)
    LinkServer = 0x31, // 링크 모듈(Server)
    BleClient = 0x32,  // BLE 클라이언트
    BleServer = 0x33,  // BLE 서버

    Range = 0x40, // 거리 센서 모듈

    Base = 0x70, // 베이스(응용 프로그램)

    ByScratch = 0x80, // 바이스크래치
    Scratch = 0x81,   // 스크래치
    Entry = 0x82,     // 네이버 엔트리

    Tester = 0xA0,    // 테스터
    Monitor = 0xA1,   // 모니터
    Updater = 0xA2,   // 펌웨어 업데이트 도구
    Encrypter = 0xA3, // 암호화 도구

    Whispering = 0xFE, // 바로 인접한 장치까지만 전달(받은 장치는 자기 자신에게 보낸 것처럼 처리하고 타 장치에 전달하지 않음)
    Broadcasting = 0xFF, // 연결된 모든 장치에 전달
}
