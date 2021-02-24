pub mod common;

//use crate::base::system::DeviceType;

pub mod protocol {
    use crate::base::system::DeviceType;

    pub enum DataType {
        None = 0x00,              // 없음
        Ping = 0x01,              // 통신 확인
        Ack = 0x02,               // 데이터 수신에 대한 응답
        Error = 0x03,             // 오류
        Request = 0x04,           // 지정한 타입의 데이터 요청
        Message = 0x05,           // 문자열 데이터
        Address = 0x06, // 장치 주소(MAC이 있는 경우 MAC) 혹은 고유번호(MAC이 없는 경우 UUID)
        Information = 0x07, // 펌웨어 및 장치 정보
        Update = 0x08,  // 펌웨어 업데이트
        UpdateLocation = 0x09, // 펌웨어 업데이트 위치 정정
        Encrypt = 0x0A, // 펌웨어 암호화
        SystemCount = 0x0B, // 시스템 카운트
        SystemInformation = 0x0C, // 시스템 정보
        Registration = 0x0D, // 제품 등록(암호화 데이터 및 등록 데이터를 데이터 길이로 구분)
        Administrator = 0x0E, // 관리자 권한 획득
        Monitor = 0x0F, // 디버깅용 값 배열 전송. 첫번째 바이트에 타입, 두 번째 바이트에 페이지 지정(수신 받는 데이터의 저장 경로 구분)
        Control = 0x10, // 조종

        Command = 0x11,           // 명령
        Pairing = 0x12,           // 페어링
        Rssi = 0x13,              // RSSI
        TimeSync = 0x14,          // 시간 동기화
        TransmissionPower = 0x15, // 전송 출력
        Configuration = 0x16,     // 설정
        Echo = 0x17, // 반향(정상적으로 송수신 되는 데이터 길이 확인용, 받은 데이터를 그대로 반환, RF로 송수신 가능한 데이터 길이를 확인할 목적으로 추가)

        Battle = 0x1F, // 전투

        // Light
        LightManual = 0x20,  // LED 수동 제어
        LightMode = 0x21,    // LED 모드 지정
        LightEvent = 0x22,   // LED 이벤트
        LightDefault = 0x23, // LED 기본 색상

        // 센서 RAW 데이터
        RawMotion = 0x30, // Motion 센서 데이터 RAW 값
        RawFlow,          // Flow 센서 데이터 RAW 값

        // 상태,  센서
        State = 0x40, // 드론의 상태(비행 모드, 방위기준, 배터리량)
        Attitude,     // 드론의 자세(Angle)(Attitude)
        Position,     // 위치
        Altitude,     // 높이, 고도
        Motion,       // Motion 센서 데이터 처리한 값(IMU)
        Range,        // 거리센서 데이터

        // 설정
        Count = 0x50,       // 카운트
        Bias,               // 엑셀, 자이로 바이어스 값
        Trim,               // 트림
        Weight,             // 무게 설정
        LostConnection,     // 연결이 끊긴 후 반응 시간 설정
        MagnetometerOffset, // 지자계 센서 오프셋 조정

        // Device
        Motor = 0x60, // 모터 제어 및 현재 제어값 확인
        MotorSingle,  // 한 개의 모터 제어
        Buzzer,       // 버저 제어
        Vibrator,     // 진동 제어

        // Input
        Button = 0x70, // 버튼
        Joystick,      // 조이스틱

        // Display
        DisplayClear = 0x80,    // 화면 지우기
        DisplayInvert,          // 화면 반전
        DisplayDrawPoint,       // 점 그리기
        DisplayDrawLine,        // 선 그리기
        DisplayDrawRect,        // 사각형 그리기
        DisplayDrawCircle,      // 원 그리기
        DisplayDrawString,      // 문자열 쓰기
        DisplayDrawStringAlign, // 문자열 쓰기
        DisplayDrawImage,       // 그림 그리기

        // Card
        CardClassify = 0x90, // 카드 색상 분류 기준 설정
        CardRange,           // 카드 색 범위(RAW 데이터의 출력 범위)
        CardRaw,             // 카드 데이터 RAW 값(유선으로만 전송)
        CardColor,           // 카드 데이터
        CardList,            // 카드 리스트 데이터
        CardFunctionList,    // 카드 함수 리스트 데이터

        // Information Assembled
        InformationAssembledForController = 0xA0, // 데이터 모음
        InformationAssembledForEntry = 0xA1,      // 데이터 모음
        InformationAssembledForByBlocks = 0xA2,   // 데이터 모음

        // Navigation
        NavigationTarget = 0xD0,   // 네비게이션 목표점
        NavigationLocation = 0xD1, // 네비게이션 드론 위치
        NavigationMonitor = 0xD2,
        NavigationHeading = 0xD3,
        NavigationCounter = 0xD4,
        NavigationSatellite = 0xD5,      // 위성 정보
        NavigationLocationAdjust = 0xD6, // 드론 위치 조정

        NavigationTargetEcef = 0xD8,   // 드론 타겟 위치(ECEF)
        NavigationLocationEcef = 0xD9, // 드론 현재 위치(ECEF)

        GpsRtkNavigationState = 0xDA,            // RTK RAW 데이터 전송
        GpsRtkExtendedRawMeasurementData = 0xDB, // RTK RAW 데이터 전송

        EndOfType,
    }

    pub enum DataType {}

    enum CommandType {
        None = 0x00, // 이벤트 없음

        Stop = 0x01, // 정지

        ModeControlFlight = 0x02, // 비행 제어 모드 설정
        Headless = 0x03,          // 헤드리스 모드 설정
        ControlSpeed = 0x04,      // 제어 속도 설정

        ClearBias = 0x05, // 자이로/엑셀 바이어스 리셋(트림도 같이 초기화 됨)
        ClearTrim = 0x06, // 트림 초기화

        FlightEvent = 0x07, // 비행 이벤트 실행

        SetDefault = 0x08,        // 기본 설정으로 초기화
        Backlight = 0x09,         // 조종기 백라이트 설정
        ModeController = 0x0A,    // 조종기 동작 모드(0x10:조종, 0x80:링크)
        Link = 0x0B,              // 링크 제어(0:Client Mode, 1:Server Mode, 2:Pairing Start)
        ClearMagnetometer = 0x0C, // 지자계 센서 초기화

        // 관리자
        ClearCounter = 0xA0, // 카운터 클리어(관리자 권한을 획득했을 경우에만 동작)
        JumpToBootloader = 0xA1, // 부트로더로 이동
        JumpToApplication = 0xA2, // 앱으로 이동

        // Navigation
        NavigationTargetClear = 0xE0,  // 네비게이션 목표점 초기화
        NavigationStart = 0xE1,        // 네비게이션 시작(처음부터)
        NavigationPause = 0xE2,        // 네비게이션 일시 정지
        NavigationRestart = 0xE3,      // 네비게이션 다시 시작(일시 정지 후 다시 시작할 때 사용)
        NavigationStop = 0xE4,         // 네비게이션 중단
        NavigationNext = 0xE5,         // 네비게이션 목표점을 다음으로 변경
        NavigationReturnToHome = 0xE6, // 시작 위치로 귀환

        GpsRtkBase = 0xEA,
        GpsRtkRover = 0xEB,

        TestLock = 0xF0, // 테스트 락(테스트를 완료하기 전까진 사용 불가 / 27:활성화, 11:해제))

        EndOfType,
    }

    pub struct Header {
        datatype: DataType,
        length: u8,
        from: DeviceType,
        to: DeviceType,
    }
}
